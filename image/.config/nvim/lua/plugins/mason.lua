return {
  "williamboman/mason.nvim",
  config = function()
    require("mason").setup()
    vim.keymap.set({ 'n' }, '<leader>fm', function() vim.lsp.buf.format() end, {})
    vim.keymap.set({ 'n', 'v' }, '<leader>rn', function() vim.lsp.buf.rename() end, {})
    vim.keymap.set({ 'n' }, '<leader>ca', function() vim.lsp.buf.code_action() end, {})
    vim.keymap.set('n', 'K', function() vim.lsp.buf.hover() end, {})
  end
}
