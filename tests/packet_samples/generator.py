import struct

header = struct.Struct('<HBBBBQfIBB')
car_motion_data = struct.Struct("<ffffffhhhhhhffffff")
packet_motion_data = struct.Struct("<4f4f4f4f4fffffffffff")

packet_event_data = struct.Struct("<4B")
fastest_lap = struct.Struct("<Bf")
buttons = struct.Struct("<I")

car_telemetry_data = struct.Struct("<H3fBbHBBH4H4B4BH4f4B")
packet_car_telemetry_data = struct.Struct("<BBb")


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


def write_telemetry_packet():
    packed_header = header.pack(2021, 1, 2, 1, 6, 1, 12.35, 123, 1, 255)
    packed_car_telemetry = packet_car_telemetry_data.pack(3, 4, 0)
    packed_telemetry_data = car_telemetry_data.pack(
        123, 1.0, 0.0, 0.0, 0, 7, 1000, 0, 50, 0, 100, 100, 100, 100, 200, 200, 200, 200, 200, 200, 200, 200, 1000, 50, 50, 50, 50,  0, 0, 0, 0)

    with open("car_telemetry.pkt", "wb") as f:
        f.write(packed_header)
        for _ in range(22):
            f.write(packed_telemetry_data)
        f.write(packed_car_telemetry)


def write_header():
    packed_header = header.pack(2021, 1, 2, 1, 0, 1, 12.35, 123, 1, 255)

    with open("header.pkt", "wb") as f:
        f.write(packed_header)


def write_event_packet_FTLP():
    packed_header = header.pack(2021, 1, 2, 1, 3, 1, 12.35, 123, 1, 255)
    packed_event_data = packet_event_data.pack(
        ord('F'), ord('T'), ord('L'), ord('P'))
    packed_fastest_lap = fastest_lap.pack(1, 10.2)

    with open("event_fltp.pkt", "wb") as f:
        f.write(packed_header)
        f.write(packed_event_data)
        f.write(packed_fastest_lap)


def write_event_packet_BUTN():
    packed_header = header.pack(2021, 1, 2, 1, 3, 1, 12.35, 123, 1, 255)
    packed_event_data = packet_event_data.pack(
        ord('B'), ord('U'), ord('T'), ord('N'))
    packed_buttons = buttons.pack(0x00001001)  # RT & A

    with open("event_butn.pkt", "wb") as f:
        f.write(packed_header)
        f.write(packed_event_data)
        f.write(packed_buttons)


def write_event_packet_SSTA():
    packed_header = header.pack(2021, 1, 2, 1, 3, 1, 12.35, 123, 1, 255)
    packed_event_data = packet_event_data.pack(
        ord('S'), ord('S'), ord('T'), ord('A'))

    with open("event_ssta.pkt", "wb") as f:
        f.write(packed_header)
        f.write(packed_event_data)


if __name__ == "__main__":
    # write_motion_packet()
    # write_header()
    # write_event_packet_FTLP()
    # write_event_packet_BUTN()
    # write_event_packet_SSTA()
    write_telemetry_packet()
