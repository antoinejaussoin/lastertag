#!/usr/bin/env bun
/** Print HTTP POSTs from the Pico 2 W on this computer's LAN address. */

import { createSocket } from "node:dgram";
import { networkInterfaces } from "node:os";

const PORT = Number(process.env.PORT ?? 8090);

function lanIp(): string {
  const probed = probeDefaultRoute();
  if (probed && !probed.startsWith("127.")) return probed;

  try {
    for (const addrs of Object.values(networkInterfaces())) {
      for (const a of addrs ?? []) {
        if (!a.internal && (a.family === "IPv4" || a.family === 4)) {
          return a.address;
        }
      }
    }
  } catch {
    // getifaddrs can fail in restricted environments
  }
  return probed ?? "127.0.0.1";
}

function probeDefaultRoute(): string | undefined {
  const socket = createSocket("udp4");
  try {
    socket.connect(80, "1.1.1.1");
    return socket.address().address;
  } catch {
    return undefined;
  } finally {
    socket.close();
  }
}

function timestamp(date = new Date()): string {
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())} ${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}`;
}

const host = lanIp();

const server = Bun.serve({
  hostname: "0.0.0.0",
  port: PORT,
  async fetch(req) {
    if (req.method !== "POST") {
      return new Response("POST JSON here\n", {
        status: 405,
        headers: { Allow: "POST" },
      });
    }

    const body = await req.text();
    const path = new URL(req.url).pathname;
    const from = server.requestIP(req)?.address ?? "?";
    console.log(`${timestamp()} POST ${path} from ${from}`);
    console.log(body);
    console.log("---");
    return new Response(null, { status: 204 });
  },
});

console.log(`Listening on 0.0.0.0:${server.port}`);
console.log("On the Pico USB console:");
console.log(`  server ${host}:${server.port}/ir`);
console.log("  save");
