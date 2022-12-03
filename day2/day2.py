import os

my_input_path = os.path.join(os.path.realpath(os.path.join(os.getcwd(), os.path.dirname(__file__))), 'data.csv')

def read_from_csv(my_input_path: str) -> list[list[int]]:

    opponent : list[list[int]] = []
    me : list[list[int]] = []
    
    with open(my_input_path) as file:
        lines = [line.rstrip() for line in file ]

    for line in lines:
        o, m = line.split()
        opponent += o
        me += m
    return opponent, me

# given
outcome = {
    'R': { 'R': 'D', 'P': 'W', 'S': 'L',},
    'P': { 'R': 'L', 'P': 'D', 'S': 'W',},
    'S': { 'R': 'W', 'P': 'L', 'S': 'D',},   
}

choice_points = { 'R': 1, 'P': 2, 'S': 3,}

outcome_points = { 'W': 6, 'L': 0, 'D': 3,}

opponent_choice = { 'A': 'R', 'B': 'P', 'C': 'S',}

opponent, me = read_from_csv(my_input_path)

# part 1
my_choice_1 = { 'X': 'R', 'Y': 'P', 'Z': 'S',}
total_score_1 = 0
for (o, m) in zip(opponent, me):
    total_score_1 += outcome_points[outcome[opponent_choice[o]][my_choice_1[m]]] + choice_points[my_choice_1[m]]

print(total_score_1) #13009

# part 2
my_choice_2 = {'X': 'L', 'Y': 'D', 'Z': 'W',}
total_score_2 = 0
for (o, m) in zip(opponent, me):
    outcome_should_be = my_choice_2[m]
    my_choices = {v: k for k, v in outcome[opponent_choice[o]].items()}
    i_should_play = my_choices[outcome_should_be]
    total_score_2 += outcome_points[my_choice_2[m]] + choice_points[i_should_play]

print(total_score_2) #10398
