# Reservoir Documentation Book

This directory contains the mdBook-based documentation for Reservoir. The documentation is built using [mdBook](https://rust-lang.github.io/mdBook/), a command line tool to create books with Markdown.

## Prerequisites

To work with the documentation, you'll need:

- [mdBook](https://rust-lang.github.io/mdBook/) installed:
  ```bash
  cargo install mdbook
  ```

- (Optional) [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid) for diagram support:
  ```bash
  cargo install mdbook-mermaid
  ```

## Building the Documentation

### Build the book

```bash
# From the book directory
mdbook build

# Or from the project root
cd book && mdbook build
```

The built documentation will be available in the `book/book` directory.

### Serve locally with live reload

```bash
# From the book directory
mdbook serve

# Or from the project root
cd book && mdbook serve
```

This will start a local server (usually at `http://localhost:3000`) with automatic rebuilding when files change.

### Watch for changes

```bash
mdbook watch
```

This will rebuild the book whenever source files change, but won't serve it.

## Structure

```
book/
├── book.toml          # Configuration file
├── src/               # Source markdown files
│   ├── SUMMARY.md     # Table of contents
│   ├── introduction.md
│   ├── installation.md
│   ├── quick-start.md
│   ├── chat-gipitty.md
│   ├── api/           # API documentation
│   ├── architecture/  # System design docs
│   ├── features/      # Feature documentation
│   ├── deployment/    # Deployment guides
│   ├── troubleshooting/ # Help and FAQ
│   └── development/   # Contributing guides
├── theme/             # Custom styling
│   └── custom.css     # Additional CSS
└── book/              # Generated output (git-ignored)
```

## Adding New Content

1. **Add a new page**: Create a new `.md` file in the appropriate `src/` subdirectory
2. **Update the table of contents**: Add the new file to `src/SUMMARY.md`
3. **Build and test**: Run `mdbook serve` to preview your changes

### Example: Adding a new feature document

```bash
# 1. Create the file
touch src/features/new-feature.md

# 2. Add content to the file
echo "# New Feature\n\nDescription of the new feature..." > src/features/new-feature.md

# 3. Add to SUMMARY.md under the Features section
# - [New Feature](./features/new-feature.md)

# 4. Test the build
mdbook serve
```

## Configuration

The book is configured in `book.toml`. Key settings:

- **Title and metadata**: Set in the `[book]` section
- **HTML output**: Configured in `[output.html]`
- **Search**: Enabled with full-text search capabilities
- **Theming**: Custom CSS in `theme/custom.css`
- **Preprocessors**: Mermaid diagrams support (when installed)

## Styling

Custom styles are defined in `theme/custom.css` and include:

- Reservoir brand colors and styling
- Improved code block appearance
- Better table formatting
- Callout boxes for notes/warnings
- Responsive design improvements
- Dark theme support

## Writing Guidelines

### Code Examples

Use appropriate language hints for syntax highlighting:

```markdown
```bash
cargo run -- start
```

```python
import reservoir
```

```json
{"model": "gpt-4", "messages": [...]}
```
```

### Callout Boxes

While mdBook doesn't have native callout support, you can use HTML:

```html
<div class="info">
💡 <strong>Tip:</strong> This is helpful information.
</div>

<div class="warning">
⚠️ <strong>Warning:</strong> Be careful with this setting.
</div>
```

### Cross-References

Link to other pages using relative paths:

```markdown
See the [Installation Guide](./installation.md) for setup instructions.
Check out the [API Reference](./api/overview.md) for detailed usage.
```

### Images

Place images in a logical subdirectory and reference them:

```markdown
![Architecture Diagram](../docs/architecture_diagram.png)
```

## Publishing

The documentation can be published to:

- **GitHub Pages**: Use the `gh-pages` branch or GitHub Actions
- **Static hosting**: Deploy the `book/` directory to any static host
- **Custom domain**: Configure in `book.toml` under `[output.html]`

### GitHub Pages Example

```yaml
# .github/workflows/docs.yml
name: Build and Deploy Documentation
on:
  push:
    branches: [ main ]
    paths: [ 'book/**' ]

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Setup mdBook
      run: |
        curl -L https://github.com/rust-lang/mdBook/releases/download/v0.4.15/mdbook-v0.4.15-x86_64-unknown-linux-gnu.tar.gz | tar xz
        echo "$(pwd)" >> $GITHUB_PATH
    - name: Build book
      run: cd book && mdbook build
    - name: Deploy to GitHub Pages
      uses: peaceiris/actions-gh-pages@v3
      with:
        github_token: ${{ secrets.GITHUB_TOKEN }}
        publish_dir: ./book/book
```

## Contributing

When contributing to the documentation:

1. Follow the existing structure and naming conventions
2. Test your changes locally with `mdbook serve`
3. Ensure all links work correctly
4. Add new pages to the table of contents in `SUMMARY.md`
5. Consider the user's journey through the documentation

## Troubleshooting

### Build Errors

- **File not found**: Check that all files referenced in `SUMMARY.md` exist
- **Broken links**: Use relative paths and verify file locations
- **CSS issues**: Check `theme/custom.css` for syntax errors

### Mermaid Diagrams Not Rendering

Install the mermaid preprocessor:
```bash
cargo install mdbook-mermaid
```

### Serve Command Fails

- Check that port 3000 isn't already in use
- Try specifying a different port: `mdbook serve --port 3001`

For more help, see the [mdBook documentation](https://rust-lang.github.io/mdBook/).