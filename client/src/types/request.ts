export enum RequestSignature {
    PING = 0,
    CREATE_ROOM = 1,
    JOIN_ROOM = 2,
    READ_ROOM = 3,
    LEAVE_ROOM = 4,
}

export enum ResponseSignature {
    ERROR = 0,
    COORDINATES_OK = 1,
    PONG = 2,
    JOIN_ROOM_SUCCESS = 3,
    CREATE_ROOM_SUCCESS = 4,
    READ_ROOM_SUCCESS = 5,
    ROOM_BROADCAST = 6,
}
