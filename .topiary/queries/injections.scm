(fenced_code_block
  (info_string
    (language) @injection.language) @info
  (code_fence_content) @injection.content
  ; mdBook inline macro
  (#not-match? @info ".*(nofmt|rust).*")
  (#not-match? @injection.content "\\{\\{\\#include")
)
