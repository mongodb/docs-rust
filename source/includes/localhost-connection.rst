If you need to run the {+server+} on your local machine for development
purposes, you must complete the following:

1. Download the `Community <https://www.mongodb.com/try/download/community>`__
   or `Enterprise <https://www.mongodb.com/try/download/enterprise>`__ version
   of the {+server+}.

#. :ref:`Install and configure <tutorials-installation>` the {+server+}.

#. Start the server.

.. important::

   Always secure your server from malicious attacks. See our
   :manual:`Security Checklist </administration/security-checklist/>` for a
   list of security recommendations.

After you successfully start the {+server+}, specify your connection
string in your driver connection code.

If your {+server+} is running locally, you can use the connection string
``"mongodb://localhost:<port>"`` where ``<port>`` is the port number you
configured your server to listen for incoming connections.

If you need to specify a different hostname or IP address, see our Server
Manual entry on :manual:`Connection Strings </reference/connection-string/>`.