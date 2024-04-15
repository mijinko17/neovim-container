return {
  "sindrets/diffview.nvim",
  dependencies = { "nvim-tree/nvim-web-devicons" },
  config = function()
    vim.keymap.set({ 'n' }, '<leader>vd', ':DiffviewOpen<CR>', {})
    vim.keymap.set({ 'n' }, '<leader>vdc', ':DiffviewClose<CR>', {})
    require("diffview").setup({
      -- use_icons = false
    })
  end
}
