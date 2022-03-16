# MCZ-1 Emulator

## What is this?

This is an emulator of the MCZ-1 computer by Zilog used for Z80 development. This is not a high fidelity emulator, I have no access to the real devices. The objective is being able to run the MCZ-1 monitor and the RIO operating system.

It uses the [iz80](https://github.com/ivanizag/iz80) library. Made with Rust.

## What was a MCZ-1 computer?

The MCZ-1 is a computer from Zilog to support the creation of new systems using their Z80 processor. It could run the RIO operating system. Other computers of the same family were the ZDS and PDS models.

Main features:

- Z80 processor
- 3Kb of [PROM](http://bitsavers.trailing-edge.com/pdf/zilog/mcz-1/firmware/ZilogPDS_3K_ROM_SOURCE.zip) with I/O, floppy and debug drivers.
- 61kb of RAM
- Dual floppy 8-inch hards ector discs. Single sided, 32 sectors, 77 tracks, 128+4 Bytes per sector. Not emulated yet.
- 1 RS232 port. Emulated with the host console.
- 2 parallet ports. Not emulated.
- 9 slots

From the Zilog [brochure](https://web.archive.org/web/20170904130919/https://amaus.org/static/S100/zilog/brochure/Zilog%20MCZ-1%20Series%20System.pdf): The Zilog MCZ-1 Microcomputer Systems are a series of general purpose computers providing very high per-
formance at a low cost. These systems are designed for high reliability and low maintenance. The MCZ-1 Systemsr
feature the use of the Z80 Microprocessor and its 158 instruction set, a disk based operating system, main stor- '
age capacity for up to 65K bytes of semiconductor memory, and two integral floppy disk drives. The MCZ-1
Series consists of units that are free standing, rack mountable and expandable beyond the basic 9 slot card cage
that is provided as part of the standard MCZ-1 Microcomputer.

![MCZ-1](doc/mcz-1.png)

## Usage examples

The MCZ-1 boots to the [monitor](http://bitsavers.trailing-edge.com/pdf/zilog/mcz-1/03-3106-01A_MCZ-1_20A_and_MCZ-1_25A_Microcomputers_Floppy_Prom_User_Guide_Dec79.pdf). Run the emulator and press any key to get the monitor prompt. Exit the emulator with control C.

```
casa@servidor:~$ ./izilogpds 
Emulation of the Zilog MCZ-1 computer
https://github.com/ivanizag/izilogpds

>REG
A  B  C  D  E  F  H  L  I  A' B' C' D' E' F' H' L'  IX   IY   PC   SP  
FF 00 00 00 00 FF 00 00 00 00 00 00 00 00 00 00 00 0000 0000 0000 FFFF 
>DUMP 0B34 40
0B34 53 45 52 49 41 4C 20 50 4F 52 54 20 49 4E 50 55 *SERIAL PORT INPU*
0B44 54 20 0D 09 42 52 45 41 4B 20 41 54 20 0B 44 49 *T ..BREAK AT .DI*
0B54 53 4B 20 45 52 52 4F 52 0D 60 40 2C 24 20 18 10 *SK ERROR.`@,$ ..*
0B64 08 04 02 01 04 02 01 03 11 00 01 3E 7F 08 00 00 *...........>....*
>DUMP 3000 10
3000 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 *................*
>SET 3000 01 02 03 04 05 06 07
>DUMP 3000 10
3000 01 02 03 04 05 06 07 00 00 00 00 00 00 00 00 00 *................*
>MOVE 3008 3000 8
>DUMP 3000 10
3000 01 02 03 04 05 06 07 00 01 02 03 04 05 06 07 00 *................*
>COMPARE 3000 3008 8
>FILL 3008 300F 8
>DUMP 3000 10
3000 01 02 03 04 05 06 07 00 08 08 08 08 08 08 08 08 *................*
>COMPARE 3000 3008 8
3000=01 3008=08 
3001=02 3009=08 
3002=03 300A=08 
3003=04 300B=08 
3004=05 300C=08 
3005=06 300D=08 
3006=07 300E=08 
3007=00 300F=08 
>OS
DISK ERROR

```

## Documentation

- [Brochure MCZ-1 Series Microcomputer System](https://web.archive.org/web/20170904130919/https://amaus.org/static/S100/zilog/brochure/Zilog%20MCZ-1%20Series%20System.pdf)
- [MCZ-1/2A and MCZ-1/25A Microcomputers Floppy PROM User Guide](http://bitsavers.trailing-edge.com/pdf/zilog/mcz-1/03-3106-01A_MCZ-1_20A_and_MCZ-1_25A_Microcomputers_Floppy_Prom_User_Guide_Dec79.pdf)
- MCZ-1 Hardware User's manual. It must exists as it is referenced in other documents, but I haven't been able to find a scan.

## References

- https://rio.early8bitz.de/index.htm
- http://www.retrotechnology.com/restore/zilog.html
- https://github.com/sebhc/sebhc/blob/master/mcz/readme.md
- http://bitsavers.trailing-edge.com/pdf/zilog/mcz-1/
- https://oldcomputers.dyndns.org/public/pub/rechner/zilog/zds/manuals/
- http://www.computinghistory.org.uk/det/12157/Zilog-Z-80-Microcomputer-System/
- https://web.archive.org/web/20170903195837/https://amaus.org/static/S100/zilog/



