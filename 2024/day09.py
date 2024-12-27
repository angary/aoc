def get_first_instance_of(pq: list[tuple[int, int]], block_size: int) -> tuple[int, int]:
    for i, v in enumerate(pq):
        if v[1] <= block_size:
            return pq.pop(i)
    

def build_disk(disk_map: str) -> list[int]:
    disk = []
    for i, c in enumerate(disk_map):
        disk += [(i // 2 if i % 2 == 0 else -1) for _ in range(int(c))]
    return disk


def checksum(disk: list[int]) -> int:
    return sum(i * val for i, val in enumerate(disk) if val != -1)


def task1(disk_map: str) -> int: 
    disk = build_disk(disk_map)
    l, r = 0, len(disk) - 1
    while l < r:
        while disk[l] != -1:
            l += 1
        if r > l:
            disk[l], disk[r] = disk[r], disk[l]
            r -= 1
    return checksum(disk)


def task2(disk_map: str) -> int:
    disk = build_disk(disk_map)
    pq = ([(i // 2, int(c)) for i, c in enumerate(disk_map) if i % 2 == 0])[::-1]
    disk = []
    for i, n in enumerate(map(int, disk_map)):
        if not pq:
            break
        # If we are odd use the current tuple
        if i % 2 == 0:
            if (i // 2, n) in pq:
                value = pq.pop(-1)
                disk.extend([value[0]] * value[1])
            else:
                disk.extend([-1] * n)
        # If we are even find the first instance of file that we can put in here
        else:
            while n > 0:
                value = get_first_instance_of(pq, n)
                if value:
                    disk.extend([value[0]] * value[1])
                    n -= value[1]
                else:
                    disk.extend([-1] * n)
                    break
    return checksum(disk)
    
def main():
    with open("input.txt") as f:
        disk_map = f.readline()
        print(task1(disk_map))
        print(task2(disk_map))


if __name__ == "__main__":
    main()