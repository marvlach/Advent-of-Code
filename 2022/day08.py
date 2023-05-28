from input import input


def read_input():

    forest = []

    for line in input():
        forest.append(list(map(int, [*line])))

    return forest


# given
forest = read_input()


def transpose(ll):
    return [list(sublist) for sublist in list(zip(*ll))]


def is_visible(forest_view):
    visible = [[True] * len(row) for row in forest]
    viewing_distance = [[0] * len(row) for row in forest]

    for i, row in enumerate(forest_view):
        moving_max = 0
        last_appear = [0] * 10
        for j, tree in enumerate(row):
            if tree > moving_max or j == 0:
                moving_max = tree

            else:
                visible[i][j] = False

            viewing_distance[i][j] = j - max(last_appear[tree:])
            last_appear[tree] = j

    return visible, viewing_distance


# left
left_visible, left_viewing_distance = is_visible(forest)

# right
right_visible_reversed, right_viewing_distance_reversed = is_visible(
    [row[::-1] for row in forest]
)
right_visible = [row[::-1] for row in right_visible_reversed]
right_viewing_distance = [row[::-1] for row in right_viewing_distance_reversed]


# top
top_visible_transposed, top_viewing_distance_transposed = is_visible(transpose(forest))
top_visible = transpose(top_visible_transposed)
top_viewing_distance = transpose(top_viewing_distance_transposed)


# bottom
(
    bottom_visible_reversed_transposed,
    bottom_viewing_distance_reversed_transposed,
) = is_visible([row[::-1] for row in transpose(forest)])
bottom_visible = transpose([row[::-1] for row in bottom_visible_reversed_transposed])
bottom_viewing_distance = transpose(
    [row[::-1] for row in bottom_viewing_distance_reversed_transposed]
)

# part 1, 2
count, score = 0, [[1] * len(row) for row in forest]
for i, row in enumerate(forest):
    for j, _ in enumerate(row):
        count = (
            count + 1
            if left_visible[i][j]
            or right_visible[i][j]
            or top_visible[i][j]
            or bottom_visible[i][j]
            else count
        )
        score[i][j] = (
            left_viewing_distance[i][j]
            * right_viewing_distance[i][j]
            * top_viewing_distance[i][j]
            * bottom_viewing_distance[i][j]
        )

print(count)  # 1849
print(max([max(row) for row in score]))  # 201600
