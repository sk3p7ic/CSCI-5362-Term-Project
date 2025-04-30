from argparse import ArgumentParser
from dataclasses import dataclass
import os

from dotenv import load_dotenv
import openai


@dataclass
class ProgramConfig:
    inputPath: str
    singleFile: bool
    keepErrors: bool
    client: openai.OpenAI
    model: str = 'gpt-4o-mini'
    outputDir: str = 'outputs'


def translate(code: str, config: ProgramConfig) -> str:
    prompt = 'Please convert the following C code to Rust code'
    if pc.keepErrors:
        prompt += ' while maintaining all memory safety errors, if possible'
    prompt += f'''
    ```c
    {code}
    ```
    Provide only the complete Rust code as a plaintext response.
    '''
    messages = [{'role': 'user', 'content': prompt}]
    response = config.client.chat.completions.create(model=config.model,
                                                     messages=messages)
    return response.choices[0].message.content.removeprefix(
        '```rs').removeprefix('```rust').removesuffix('```').strip()


def get_output_name(fname: str, config: ProgramConfig) -> str:
    if config.keepErrors:
        fname = 'unsure-' + fname
    fname = fname.replace('.c', '.rs')
    return f'{config.outputDir}/{fname}'


def save(code: str, fname: str, config: ProgramConfig):
    if os.path.exists(config.outputDir) and \
            not os.path.isdir(config.outputDir):
        return Exception('Output path is not a directory.')
    if not os.path.exists(config.outputDir):
        os.mkdir(config.outputDir)
    path = get_output_name(fname, config)
    with open(path, 'w') as f:
        f.write(code)
    print(f'Saved code to \'{path}\'.')


def translate_and_save(fname: str, fpath: str, config: ProgramConfig):
    if str(os.path.splitext(fname)[1].lower()) != '.c':
        print(f'{fname} is not a C file. Skipping.')
        return
    print('Translating \'{}\' -> \'{}\''.format(pc.inputPath,
                                                get_output_name(fname,
                                                                pc)))
    with open(f'{fpath}/{fname}', 'r') as f:
        code = ''.join(f.readlines())
    rust = translate(code, pc)
    save(rust, fname, config)


if __name__ == '__main__':
    load_dotenv('.env')
    parser = ArgumentParser(prog='C to Rust Translator',
                            description='C to Rust translator using ChatGPT',
                            epilog='For CSCI 5362, Spring 2025 @ TAMU-SA.')
    parser.add_argument('filename')
    parser.add_argument('-k', '--keep', action='store_true',
                        help='Whether memory errors are desired to be kept.')
    args = parser.parse_args()

    api_key = os.environ['OPENAI_API_KEY']
    pc = ProgramConfig(inputPath=args.filename,
                       singleFile=os.path.isfile(args.filename),
                       client=openai.OpenAI(api_key=api_key),
                       keepErrors=args.keep)
    if pc.singleFile:
        fname = os.path.basename(pc.inputPath)
        translate_and_save(fname,
                           pc.inputPath.removesuffix(fname),
                           pc)
    else:
        for fname in os.listdir(pc.inputPath):
            if os.path.isdir(fname) or fname.endswith('.md'):
                continue
            translate_and_save(fname, pc.inputPath, pc)
