# CSCI-5362-Term-Project

## Dependencies

Install the python requirements with
```bash
pip3 install -r requirements.txt
```

Please also create an OpenAI API key and set it in your `.env` file. A
`.env.example` file is present as an example of how this API key would be set.
```bash
cp .env.example .env
# Then edit the file to use your key, replacing all <bracketed> content.
```

## Performing Translation

To translate C code, run
```bash
python3 converter.py <c-file-path>
```

To force-keep memory errors in the C code (if possible), run:
```bash
python3 converter.py -k <c-file-path>
```

Directory paths may also be used. **All C files within that directory will then
be translated.**
