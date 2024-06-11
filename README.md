# Buffer Overflow Scripts & Learning

## Useful Links
- TryHackMe OSCP Prep: <https://tryhackme.com/r/room/bufferoverflowprep>
- Tib3rius Buffer Overflow Guide: <https://github.com/Tib3rius/Pentest-Cheatsheets/blob/master/exploits/buffer-overflows.rst>

## OSCP Prep on THM

### Access the Windows machine
- RDP access to the Windows machine (On Archlinux with Rdesktop)
```sh
rdesktop -u admin -p password 10.10.18.138:3389
```

### On the Windows target
- Start Immunity Debugger on the Windows machine and open the OSCP exe file
- Run the exe in debug mode

### On our box
- Now able to call the binary (exposed on port 1337) from the attacker's box with:
```sh
nc 10.10.18.138 1337
```

### Buffer Overflow Steps

#### 1. Fuzzing
#### 2. Crash Replication & Controlling EIP
#### 3. Finding Bad Characters
#### 4. Finding a Jump Point
#### 5. Generate Payload
#### 6. Prepend NOPs
#### 7. Exploit
