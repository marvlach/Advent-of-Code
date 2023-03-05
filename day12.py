from input import input


def ij_to_n(i: int, j: int, N: int, M: int):
    return M * i + j


def n_to_ij(n: int, N: int, M: int):
    return n // M, n % N


def read_input(part: int):

    heightmap: list[list[str]] = []
    for line in input():
        heightmap.append(list(line))

    N: int = len(heightmap)
    M: int = len(heightmap[0])

    vertex: list[list[int | None]] = []
    source: list[int] = []
    target: int
    for i, v_i in enumerate(heightmap):
        for j, _ in enumerate(v_i):
            n = ij_to_n(i, j, N, M)
            if heightmap[i][j] == "S":
                heightmap[i][j] = "a"
                if part == 1:
                    source.append(n)

            if heightmap[i][j] == "E":
                heightmap[i][j] = "z"
                target = n

            if part == 2 and heightmap[i][j] == "a":
                source.append(n)

    for i, v_i in enumerate(heightmap):
        for j, _ in enumerate(v_i):
            n = ij_to_n(i, j, N, M)
            vertex.append([None] * 4)
            current = heightmap[i][j]

            # 0 up
            if i - 1 >= 0 and ord(heightmap[i - 1][j]) - ord(current) <= 1:
                vertex[n][0] = ij_to_n(i - 1, j, N, M)

            # 1 right
            if j + 1 < M and ord(heightmap[i][j + 1]) - ord(current) <= 1:
                vertex[n][1] = ij_to_n(i, j + 1, N, M)

            # 2 down
            if i + 1 < N and ord(heightmap[i + 1][j]) - ord(current) <= 1:
                vertex[n][2] = ij_to_n(i + 1, j, N, M)

            # 3 left
            if j - 1 >= 0 and ord(heightmap[i][j - 1]) - ord(current) <= 1:
                vertex[n][3] = ij_to_n(i, j - 1, N, M)

    return vertex, N, M, source, target


def bfs(vertex, source: list[int], N, M):
    max_dist: int = N * M
    dist: list[int] = [max_dist] * (N * M)
    visited: list[int] = [False] * (N * M)
    queue = []

    for s in source:
        dist[s] = 0
        visited[s] = True
        queue.append(s)

    while queue:
        current = queue.pop(0)

        for adj in vertex[current]:
            if adj is None:
                continue

            if not visited[adj]:
                visited[adj] = True
                queue.append(adj)

            if dist[adj] > dist[current] + 1:
                dist[adj] = dist[current] + 1

    return dist


vertex, N, M, source, target = read_input(1)
dist = bfs(vertex, source, N, M)
print(dist[target])  # 447

vertex, N, M, source, target = read_input(2)
dist = bfs(vertex, source, N, M)
print(dist[target])  # 446