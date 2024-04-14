vim.g.clipboard = {
  name = 'named_pipe',
  copy = {
    ["+"] = "copy.sh",
    ["*"] = "copy.sh",
  },
  paste = {
    ["+"] = { 'paste.sh' },
    ["*"] = { 'paste.sh' },
  },
  cache_enabled = false,
}
