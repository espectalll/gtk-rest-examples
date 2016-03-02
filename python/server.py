#!/usr/bin/env python

import cherrypy

class Server(object):
    exposed = True

    @cherrypy.tools.accept(media='text/plain')

    def POST(self):
        return "Gotcha! Here's your data back: {}".format(
            cherrypy.request.body.read(
                int(cherrypy.request.headers['Content-Length'])))

if __name__ == '__main__':
    conf = {
        '/': {
            'request.dispatch': cherrypy.dispatch.MethodDispatcher()
        }
    }
    cherrypy.config.update({'server.socket_port': 80})
    cherrypy.quickstart(Server(), '/', conf)
