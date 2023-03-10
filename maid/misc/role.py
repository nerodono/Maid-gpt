from enum import Enum


class HouseRole(str, Enum):
    master = "master"
    guest = "guest"

    def refer(self):
        match self:
            case HouseRole.master:
                return "Your master"
            case HouseRole.guest:
                return "Your master's guest"
