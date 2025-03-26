just := env_var_or_default("JUST", "just")
just_dir := env_var_or_default("JUST_DIR", justfile_directory())

python := env_var_or_default("PYTHON", "python3")
cargo := env_var_or_default("CARGO", "cargo")

publish_domain := env_var_or_default("PUBLISH_DOMAIN", "component-model.bytecodealliance.org")

scripts_dir := env_var_or_default("SCRIPTS_DIR", "scripts")

sitemap_output_path := env_var_or_default("SITEMAP_OUTPUT_PATH", absolute_path("./component-model/book/html/sitemap.xml"))
book_output_dir := env_var_or_default("BOOK_OUTPUT_DIR", "./component-model/book/html")

@_default:
    {{just}} --list

# Build the sitemap
@build-sitemap:
    {{python}} {{scripts_dir}}/generate_sitemap.py --domain "{{publish_domain}}" --higher-priority "design" --output-path {{sitemap_output_path}}
    if [[ ! -f "{{book_output_dir}}/index.html" ]]; then \
      echo "[error] index.html @ [{{book_output_dir}}] is missing. Build or path misconfigured"; \
      exit 1; \
    fi


