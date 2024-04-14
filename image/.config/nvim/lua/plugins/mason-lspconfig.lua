return {
  "williamboman/mason-lspconfig.nvim",
  dependencies = { "williamboman/mason.nvim" },
  after = 'mason.nvim',
  config = function()
    require("mason-lspconfig").setup()
  end
}
