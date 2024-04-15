return {
  "NeogitOrg/neogit",
  after = 'diffview.nvim',
  config = function()
    local neogit = require('neogit')
    neogit.setup {}
  end
}

