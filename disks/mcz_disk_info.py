# Usage: python3 l2bin.py
import sys

disk = open(sys.argv[1], 'rb')
data = disk.read()
for seq_track in range(0, 77):
    for seq_sector in range(0, 32):
        start = (seq_track * 32 + seq_sector)* 136
        sector = data[start] - 128
        track = data[start+1]
        back_sector = data[start+130]
        back_track = data[start+131]
        forw_sector = data[start+132]
        forw_track = data[start+133]
        crc = data[start+134] + data[start+135]*256
        print ('T:{:02} S:{:02} B:{:02}-{:02} F:{:02}-{:02} CRC:{}'.format(track, sector, back_track, back_sector, forw_track, forw_sector, crc))
        if seq_sector != sector:
            print ('Warning: Sector number mismatch {:02} in file, {:02} by position'.format(sector, seq_sector))
        if seq_track != track:
            print ('Warning: Track number mismatch {:02} in file, {:02} by position'.format(track, seq_track))
disk.close()