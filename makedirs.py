import os

root = 'src/solutions/y_2022'
inputs_root = 'inputs/2022'

for day in range(1, 26):
    if os.path.exists(f'{root}/day_{day:02d}.rs'):
        print(f'Already exists: day_{day:02d}')
    else:
        print(f'Creating: day_{day:02d}')
        with open(f'{root}/day_{day:02d}.rs', 'w') as f:
            with open('src/solutions/template.rs') as t:
                f.write(t.read().replace('XX', f'{day:02d}'))
        os.makedirs(f'{inputs_root}/day_{day:02d}', exist_ok=True)
        open(f'{inputs_root}/day_{day:02d}/input.txt', 'w').close()
        open(f'{inputs_root}/day_{day:02d}/test.txt', 'w').close()
        
                
with open(f'{root}/mod.rs', 'w') as f:
    f.write('use crate::Answer;\n\n')
    for day in range(1, 26):
        f.write(f'mod day_{day:02d};\r')
    f.write('\n\n')
    f.write(f'pub const ALL: [&dyn Answer; 25] = [\n')
    for day in range(1, 26):
        f.write(f'    &day_{day:02d}::Day{day:02d},\r')
    f.write('\n];\n')