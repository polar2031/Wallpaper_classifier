import subprocess
import os
import sys
from PIL import Image
import argparse


def input_path(string):
    path = os.path.realpath(string)
    if not os.path.isdir(path):
        msg = 'No such directory: %r' % string
        raise argparse.ArgumentTypeError(msg)
    return path


def make_test_func(args):

    test_list = []
    if args.l:
        if args.s:
            test_list.append(lambda x, y: x >= y)
        else:
            test_list.append(lambda x, y: x > y)
    elif args.v:
        if args.s:
            test_list.append(lambda x, y: x <= y)
        else:
            test_list.append(lambda x, y: x < y)

    if args.wg:
        test_list.append(lambda x, y: x >= args.wg)
    elif args.w:
        test_list.append(lambda x, y: x == args.w)
    elif args.wl:
        test_list.append(lambda x, y: x <= args.wl)

    if args.hg:
        test_list.append(lambda x, y: y >= args.hg)
    elif args.h:
        test_list.append(lambda x, y: y == args.h)
    elif args.hl:
        test_list.append(lambda x, y: y <= args.hl)

    def inner_func(w, h):
        result = True
        for t in test_list:
            result = result and t(w, h)
        return result

    return inner_func


if __name__ == '__main__':
    img_type_list = [".jpg", ".jpeg", ".png", ".bmp"]
    working_path = os.path.dirname(sys.executable)
    os.chdir(working_path)
    parser = argparse.ArgumentParser(prog='Image classifier', add_help=False)
    parser.add_argument('input_path', nargs='?', type=input_path, default=working_path, help='Path (folder) of images')
    parser.add_argument('-help', action='help', help='show this help message and exit')
    shape_option_group = parser.add_argument_group(title='Shape Option',
                                                   description='Select Square/Vertical/Horizontal image. '
                                                               'Option -s can be used with -v/-l')
    shape_option_group.add_argument('-s', action='store_true', help='Square')
    shape_option_group = shape_option_group.add_mutually_exclusive_group()
    shape_option_group.add_argument('-v', action='store_true', help='Vertical')
    shape_option_group.add_argument('-l', action='store_true', help='Landscape (horizontal)')
    width_option_group = parser.add_argument_group(title='Width Option')
    width_option_group = width_option_group.add_mutually_exclusive_group()
    width_option_group.add_argument('-w', type=int, help='Width is equal to W')
    width_option_group.add_argument('-wg', type=int, help='Width is equal to or greater than WG')
    width_option_group.add_argument('-wl', type=int, help='Width is equal to or less than WL')
    height_option_group = parser.add_argument_group(title='Height Option')
    height_option_group = height_option_group.add_mutually_exclusive_group()
    height_option_group.add_argument('-h', type=int, help='Height is equal to H')
    height_option_group.add_argument('-hg', type=int, help='Height is equal to or greater than HG')
    height_option_group.add_argument('-hl', type=int, help='Height is equal to or greater than HL')
    processing_option_group = parser.add_argument_group(title='Process for matching image(s)')
    processing_option_group = processing_option_group.add_mutually_exclusive_group()
    processing_option_group.add_argument('-mv', type=os.path.realpath, help='Move matched images to MV')
    processing_option_group.add_argument('-cp', type=os.path.realpath, help='Copy matched images to CP')
    processing_option_group.add_argument('-rm', action='store_true', help='Remove matched images')

    # parse argument
    args = parser.parse_args()

    # prepare output folder
    output_path = args.mv or args.cp
    if output_path:
        try:
            os.mkdir(output_path)
        except FileExistsError:
            pass
        except Exception as e:
            print(e)
            sys.exit(-1)

    # get image file list
    image_entry_list = []
    with os.scandir(args.input_path) as it:
        for entry in it:
            if not entry.name.startswith('.') and \
                    entry.is_file() and \
                    os.path.splitext(entry.name)[-1] in img_type_list:
                image_entry_list.append(entry)

    test = make_test_func(args)
    matched_image_entry_list = []
    for image_entry in image_entry_list:
        img = Image.open(image_entry.path)
        width, height = img.size
        if test(width, height):
            matched_image_entry_list.append(image_entry)

    if len(matched_image_entry_list) == 0:
        print('No image matched')
        sys.exit(0)

    for image_entry in matched_image_entry_list:
        if args.mv:
            subprocess.run(['mv', '-f', image_entry.path, output_path])
            print('Move ' + image_entry.path + ' to ' + output_path)
        elif args.cp:
            subprocess.run(['cp', '-f', image_entry.path, output_path])
            print('Copy ' + image_entry.path + ' to ' + output_path)
        elif args.rm:
            subprocess.run(['rm', '-I', image_entry.path])
            print('Remove ' + image_entry.path)
        else:
            print(image_entry.path)
    sys.exit(0)
