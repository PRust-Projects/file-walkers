import argparse
import os
import pathlib
import random
import uuid


def gen_dirs(directory, num_dirs):
    directories_generated = []

    for x in range(num_dirs):
        dirname = os.path.join(directory, str(uuid.uuid4()))
        directories_generated.append(dirname)
        os.makedirs(dirname)

    return directories_generated


def gen_files(directory, num_files):
    files_generated = []

    for x in range(num_files):
        filename = os.path.join(directory, str(uuid.uuid4()))
        files_generated.append(filename)
        with open(filename, 'a') as file_handler:
            file_handler.write('')

    return files_generated


def gen_narrow_tree(root_path, depth):
    max_dirs_per_depth = int(depth / 5) or 1

    directories_to_process = [root_path]
    for i in range(depth):
        for base_path in directories_to_process:
            new_directories_to_process = []
            num_dirs = random.randint(1, max_dirs_per_depth)

            dirs_generated = gen_dirs(base_path, num_dirs)
            for directory in dirs_generated:
                new_directories_to_process.append(directory)
                num_files = random.randint(0, 5)
                if num_files > 0:
                    gen_files(directory, num_files)

        directories_to_process = new_directories_to_process


def gen_wide_tree(root_path, depth):
    max_dirs_per_depth = int(depth / 2) or 1

    directories_to_process = [root_path]
    for i in range(depth):
        for base_path in directories_to_process:
            new_directories_to_process = []
            num_dirs = random.randint(1, max_dirs_per_depth)

            dirs_generated = gen_dirs(base_path, num_dirs)
            for directory in dirs_generated:
                new_directories_to_process.append(directory)
                num_files = random.randint(0, 20)
                if num_files > 0:
                    gen_files(directory, num_files)

        directories_to_process = new_directories_to_process
    pass


def main():
    parser = argparse.ArgumentParser(description="Generate a directory tree for testing purposes")
    parser.add_argument("tree-format", type=str, choices=['wide', 'narrow'],
                        help="Specify the type of tree to be generated (support for 'wide' and 'narrow')")
    parser.add_argument("--depth", type=int, default=5,
                        help="Specify the depth of the directory tree to generate.")
    parser.add_argument("--root-path", type=str, default=pathlib.Path.home(),
                        help="Specify the root path for the directory tree.")

    args = vars(parser.parse_args())
    root_path = args['root_path']
    depth = args['depth']
    
    if args['tree-format'] == 'wide':
        gen_wide_tree(root_path, depth)
    elif args['tree-format'] == 'narrow':
        gen_narrow_tree(root_path, depth)

if __name__ == '__main__':
    main()
