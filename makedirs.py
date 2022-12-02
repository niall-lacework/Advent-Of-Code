import os

root = 'src/solutions/y_2022'
inputs_root = 'inputs/2022'

for day in range(1, 26):
    if os.path.exists(f'{root}/day_{day:02d}.rs'):
        print(f'Already exists: day_{day:02d}')
    else:
        print(f'Creating: day_{day:02d}')