#!/usr/bin/env julia

using Merly

server = Merly.app()

@route POST "/" begin
    "Gotcha! Here's your data back: " + body
end

server.start("localhost", 80)
