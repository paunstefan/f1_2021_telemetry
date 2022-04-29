import struct

header = struct.Struct('<HBBBBQfIBB')
car_motion_data = struct.Struct("<ffffffhhhhhhffffff")
packet_motion_data = struct.Struct("<4f4f4f4f4fffffffffff")

packet_event_data = struct.Struct("<4B")
fastest_lap = struct.Struct("<Bf")
buttons = struct.Struct("<I")


def write_motion_packet():
    packed_header = header.pack(2021, 1, 2, 1, 0, 1, 12.35, 123, 1, 255)
    packed_car_motion = car_motion_data.pack(
        1.0, 2.0, 3.0, 10.0, 20.0, 30.0, 2, 2, 2, 3, 3, 3, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0)
    packed_motion_data = packet_motion_data.pack(
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0)

    with open("motion.pkt", "wb") as f:
        f.write(packed_header)
        for _ in range(22):
            f.write(packed_car_motion)
        f.write(packed_motion_data)


def write_header():
    packed_header = header.pack(2021, 1, 2, 1, 0, 1, 12.35, 123, 1, 255)

    with open("header.pkt", "wb") as f:
        f.write(packed_header)


def write_event_packet_FLTP():
    packed_header = header.pack(2021, 1, 2, 1, 0, 1, 12.35, 123, 1, 255)
    packed_event_data = packet_event_data.pack(
        ord('F'), ord('L'), ord('T'), ord('P'))
    packed_fastest_lap = fastest_lap.pack(1, 10.2)

    with open("event_fltp.pkt", "wb") as f:
        f.write(packed_header)
        f.write(packed_event_data)
        f.write(packed_fastest_lap)


def write_event_packet_BUTN():
    packed_header = header.pack(2021, 1, 2, 1, 0, 1, 12.35, 123, 1, 255)
    packed_event_data = packet_event_data.pack(
        ord('B'), ord('U'), ord('T'), ord('N'))
    packed_buttons = buttons.pack(0x00001000)  # R2

    with open("event_butn.pkt", "wb") as f:
        f.write(packed_header)
        f.write(packed_event_data)
        f.write(packed_buttons)


if __name__ == "__main__":
    # write_motion_packet()
    # write_header()
    write_event_packet_FLTP()
    write_event_packet_BUTN()
