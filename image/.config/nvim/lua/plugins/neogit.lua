return {
  "NeogitOrg/neogit",
  dependencies = {"nvim-lua/plenary.nvim", "sindrets/diffview.nvim", "lewis6991/gitsigns.nvim" }, -- gitsigns must be loaded before neogit, otherwise start screen disappears.
  config = true
}

