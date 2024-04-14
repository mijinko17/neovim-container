return {
  "jose-elias-alvarez/null-ls.nvim",
  requires = { "nvim-lua/plenary.nvim" },
  config = function()
    local null_ls = require("null-ls")
    null_ls.setup()
  end
}
