# F1 2021 Telemetry

The Codemasters F1 game series includes a feature to output telemetry info from the game in real time. 
This telemetry can be used as input for other programs, such as controllers, statistics apps, telemetry displays like the ones
you can see during real F1 races.

This library implements a decoder for the F1 2021 UDP packets.

## Packet types

Each piece of information about the race is sent as a different packet type, this library implements the following types.
All data is encoded in Little Endian format.

### Packet ID

The packet IDs are as follows:

| Packet Name          | Value | Description                                                                      |
| -------------------- | ----- | -------------------------------------------------------------------------------- |
| Motion               | 0     | Contains all motion data for player’s car – only sent while player is in control |
| Session              | 1     | Data about the session – track, time left                                        |
| Lap Data             | 2     | Data about all the lap times of cars in the session                              |
| Event                | 3     | Various notable events that happen during a session                              |
| Participants         | 4     | List of participants in the session, mostly relevant for multiplayer             |
| Car Setups           | 5     | Packet detailing car setups for cars in the race                                 |
| Car Telemetry        | 6     | Telemetry data for all cars                                                      |
| Car Status           | 7     | Status data for all cars                                                         |
| Final Classification | 8     | Final classification confirmation at the end of a race                           |
| Lobby Info           | 9     | Information about players in a multiplayer lobby                                 |
| Car Damage           | 10    | Damage status for all cars                                                       |
| Session History      | 11    | Lap and tyre data for session                                                    |

### Packet header

```c
struct PacketHeader
{
    uint16    m_packetFormat;            // 2021
    uint8     m_gameMajorVersion;        // Game major version - "X.00"
    uint8     m_gameMinorVersion;        // Game minor version - "1.XX"
    uint8     m_packetVersion;           // Version of this packet type,
                                         // all start from 1
    uint8     m_packetId;                // Identifier for the packet type,
                                         // see below
    uint64    m_sessionUID;              // Unique identifier for the session
    float     m_sessionTime;             // Session timestamp
    uint32    m_frameIdentifier;         // Identifier for the frame the data
                                         // was retrieved on
    uint8     m_playerCarIndex;          // Index of player's car in the array
    uint8     m_secondaryPlayerCarIndex; // Index of secondary player's car in
                                         // the array (split-screen)
                                         // 255 if no second player
};
```

### Motion packets

Motion packets contain physics data for all cars in the race, with additional data for the player car.

Size: 1464 bytes

```c
struct CarMotionData
{
    float         m_worldPositionX;           // World space X position
    float         m_worldPositionY;           // World space Y position
    float         m_worldPositionZ;           // World space Z position
    float         m_worldVelocityX;           // Velocity in world space X
    float         m_worldVelocityY;           // Velocity in world space Y
    float         m_worldVelocityZ;           // Velocity in world space Z
    int16         m_worldForwardDirX;         // World space forward X direction (normalised)
    int16         m_worldForwardDirY;         // World space forward Y direction (normalised)
    int16         m_worldForwardDirZ;         // World space forward Z direction (normalised)
    int16         m_worldRightDirX;           // World space right X direction (normalised)
    int16         m_worldRightDirY;           // World space right Y direction (normalised)
    int16         m_worldRightDirZ;           // World space right Z direction (normalised)
    float         m_gForceLateral;            // Lateral G-Force component
    float         m_gForceLongitudinal;       // Longitudinal G-Force component
    float         m_gForceVertical;           // Vertical G-Force component
    float         m_yaw;                      // Yaw angle in radians
    float         m_pitch;                    // Pitch angle in radians
    float         m_roll;                     // Roll angle in radians
};

struct PacketMotionData
{
    PacketHeader    m_header;                 // Header

    CarMotionData   m_carMotionData:[22];     // Data for all cars on track
                                                // Extra player car ONLY data
    float         m_suspensionPosition[4];       // Note: All wheel arrays have the following order:
    float         m_suspensionVelocity[4];       // RL, RR, FL, FR
    float         m_suspensionAcceleration[4];   // RL, RR, FL, FR
    float         m_wheelSpeed[4];               // Speed of each wheel
    float         m_wheelSlip[4];                // Slip ratio for each wheel
    float         m_localVelocityX;              // Velocity in local space
    float         m_localVelocityY;              // Velocity in local space
    float         m_localVelocityZ;              // Velocity in local space
    float         m_angularVelocityX;            // Angular velocity x-component
    float         m_angularVelocityY;            // Angular velocity y-component
    float         m_angularVelocityZ;            // Angular velocity z-component
    float         m_angularAccelerationX;        // Angular velocity x-component
    float         m_angularAccelerationY;        // Angular velocity y-component
    float         m_angularAccelerationZ;        // Angular velocity z-component
    float         m_frontWheelsAngle;            // Current front wheels angle in radians
};

```

### Event packets

Event packets contain data for events happening during the race. They don't have a set rate, and will 
only be sent as needed.

```c
// The event details packet is different for each type of event.
// Make sure only the correct type is interpreted.
union EventDataDetails
{
    struct
    {
        uint8	vehicleIdx; // Vehicle index of car achieving fastest lap
        float	lapTime;    // Lap time is in seconds
    } FastestLap;

    struct
    {
        uint8   vehicleIdx; // Vehicle index of car retiring
    } Retirement;

    struct
    {
        uint8   vehicleIdx; // Vehicle index of team mate
    } TeamMateInPits;

    struct
    {
        uint8   vehicleIdx; // Vehicle index of the race winner
    } RaceWinner;

    struct
    {
    	uint8 penaltyType;		// Penalty type – see Appendices
        uint8 infringementType;		// Infringement type – see Appendices
        uint8 vehicleIdx;         	// Vehicle index of the car the penalty is applied to
        uint8 otherVehicleIdx;    	// Vehicle index of the other car involved
        uint8 time;               	// Time gained, or time spent doing action in seconds
        uint8 lapNum;             	// Lap the penalty occurred on
        uint8 placesGained;       	// Number of places gained by this
    } Penalty;

    struct
    {
        uint8 vehicleIdx;		// Vehicle index of the vehicle triggering speed trap
        float speed;      		// Top speed achieved in kilometres per hour
        uint8 overallFastestInSession;   // Overall fastest speed in session = 1, otherwise 0
        uint8 driverFastestInSession;    // Fastest speed for driver in session = 1, otherwise 0
    } SpeedTrap;

    struct
    {
        uint8 numLights;		// Number of lights showing
    } StartLIghts;

    struct
    {
        uint8 vehicleIdx;                 // Vehicle index of the vehicle serving drive through
    } DriveThroughPenaltyServed;

    struct
    {
        uint8 vehicleIdx;                 // Vehicle index of the vehicle serving stop go
    } StopGoPenaltyServed;

    struct
    {
        uint32 flashbackFrameIdentifier;  // Frame identifier flashed back to
        float flashbackSessionTime;       // Session time flashed back to
    } Flashback;

    struct
    {
        uint32         m_buttonStatus;    // Bit flags specifying which buttons are being pressed
                                          // currently - see appendices
    } Buttons;
};

struct PacketEventData
{
    PacketHeader    	m_header;               	// Header

    uint8           	m_eventStringCode[4];   	// Event string code, see below
    EventDataDetails	m_eventDetails;         	// Event details - should be interpreted differently
                                                 // for each type
};
```

Event codes:

| Event                | Code   | Description                                    |
| -------------------- | ------ | ---------------------------------------------- |
| Session Started      | “SSTA” | Sent when the session starts                   |
| Session Ended        | “SEND” | Sent when the session ends                     |
| Fastest Lap          | “FTLP” | When a driver achieves the fastest lap         |
| Retirement           | “RTMT” | When a driver retires                          |
| DRS enabled          | “DRSE” | Race control have enabled DRS                  |
| DRS disabled         | “DRSD” | Race control have disabled DRS                 |
| Team mate in pits    | “TMPT” | Your team mate has entered the pits            |
| Chequered flag       | “CHQF” | The chequered flag has been waved              |
| Race Winner          | “RCWN” | The race winner is announced                   |
| Penalty Issued       | “PENA” | A penalty has been issued – details in event   |
| Speed Trap Triggered | “SPTP” | Speed trap has been triggered by fastest speed |
| Start lights         | “STLG” | Start lights – number shown                    |
| Lights out           | “LGOT” | Lights out                                     |
| Drive through served | “DTSV” | Drive through penalty served                   |
| Stop go served       | “SGSV” | Stop go penalty served                         |
| Flashback            | “FLBK” | Flashback activated                            |
| Button status        | “BUTN” | Button status changed                          |


### TODO: Other packet types