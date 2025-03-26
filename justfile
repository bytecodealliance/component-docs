just := env_var_or_default("JUST", "just")
just_dir := env_var_or_default("JUST_DIR", justfile_directory())

python := env_var_or_default("PYTHON", "python3")
cargo := env_var_or_default("CARGO", "cargo")

publish_domain := env_var_or_default("PUBLISH_DOMAIN", "component-model.bytecodealliance.org")

scripts_dir := env_var_or_default("SCRIPTS_DIR", "scripts")

sitemap_output_path := absolute_path("./component-model/book/html/sitemap.xml")

@_default:
    {{just}} --list

# Build the sitemap
@build-sitemap:
      {{python}} {{scripts_dir}}/generate_sitemap.py --domain "{{publish_domain}}" --higher-priority "design" --output-path {{sitemap_output_path}}

