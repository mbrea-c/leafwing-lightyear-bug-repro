#!/usr/bin/env python3

import subprocess
from typing import IO, List
import threading


class GameProc:
    def __init__(self, name: str, command: List[str], padding: int = 8, env={}):
        self.name = name
        self.command = command
        self.padding = padding
        self.env = env

    def go(self):
        self.proc = subprocess.Popen(
            self.command,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
        )
        self.thread = threading.Thread(target=self.process_output, args=())
        self.thread.start()

    def process_output(self):
        name_len = min(len(self.name), self.padding - 2)
        padding = self.padding - name_len

        prefix = f"[{self.name[:name_len]}]" + (" " * padding)

        with self.proc.stdout:
            for line in iter(self.proc.stdout.readline, b""):
                output = line.decode("utf-8").rstrip()
                print(f"{prefix}|{output}")
        self.proc.stdout.close()

    def join(self):
        self.proc.wait()
        self.thread.join()

    def kill(self):
        print(f"Killing process {self.name}")
        self.proc.kill()
        self.thread.join()


processes = [
    GameProc(
        name="SERVER",
        command=["cargo", "run", "--", "server"],
        env={
            "RUST_BACKTRACE": "1",
        },
    ),
    GameProc(
        name="CLIENT",
        command=[
            "cargo",
            "run",
            "--",
            "client",
        ],
        env={
            "RUST_BACKTRACE": "1",
        },
    ),
    # GameProc(
    #     "CLIENT 2",
    #     [
    #         "cargo",
    #         "run",
    #         "--release",
    #         "--",
    #         "client",
    #         "--ip",
    #         "127.0.0.1",
    #         "--port",
    #         "1337",
    #     ],
    #     env={
    #         "RUST_BACKTRACE": "1",
    #     },
    # ),
]

try:
    for proc in processes:
        proc.go()
    for proc in processes:
        proc.join()
except:
    for proc in processes:
        proc.kill()
