# NOTES

## Setting up GROQ_API_KEY

### For Production (Cloudflare Workers):

```sh
wrangler secret put GROQ_API_KEY
# Enter your actual Groq API key when prompted
```

### For Local Development:

```sh
# Option 1: Use .dev.vars file (recommended)
# Create .dev.vars file in project root:
echo "GROQ_API_KEY=your_actual_api_key_here" > .dev.vars

# Option 2: Use environment variable with wrangler dev
GROQ_API_KEY=your_actual_api_key_here wrangler dev

# Option 3: Export environment variable
export GROQ_API_KEY=your_actual_api_key_here
wrangler dev
```

### Important:

- Add `.dev.vars` to your `.gitignore` to avoid committing secrets
- The `.dev.vars` file is automatically loaded by `wrangler dev`
