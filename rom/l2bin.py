# Usage: python3 l2bin.py

source = open('MCZ.PROM.78089.L', 'r')
dest = open('MCZ.PROM.78089.BIN', 'wb')
next = 0
for line in source:
    # Useful lines are like: 0013   210000
    if len(line) >= 8 and line[0] != ' ' and line[7] != ' ':
        parts = line.split()
        try:
            address = int(parts[0], 16)
            code = bytes.fromhex(parts[1]) 
            while next < address:
                dest.write(b'\x00')
                next += 1

            # Fix pruned trings in .L
            if address == 0x0002:
                code = '78089N'.encode('ascii')
            elif address == 0x0b34:
                code = 'SERIAL PORT INPUT '.encode('ascii')
            elif address == 0x0b48:
                code = 'BREAK AT '.encode('ascii')
            elif address == 0x0b52:
                code = 'DISK ERROR'.encode('ascii')
            elif address == 0x0b87:
                code = "A B C D E F H L I A'B'C'D'".encode('ascii')
            elif address == 0x0ba1:
                code = "E'F'H'L'IXIYPCSP".encode('ascii')

            dest.write(code)
            next += len(code)
        except ValueError:
            continue

dest.close()
source.close()