#!/usr/bin/env julia

#=
Right now, I have no idea at all if Meryl supports
any other parsing than the default XML one for POST
It doesn't seem to care about the data's header...

So yep. Right now, only send XML-written stuff.
=#

using Merly
using JSON

server = Merly.app()

@route POST "/" begin
    "Gotcha! Here's your data back: $(JSON.json(body))"
end

server.start("localhost", 80)
