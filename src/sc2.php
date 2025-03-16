<?php

$socket_file = "/tmp/chart_comm"; // Replace with the actual path to your Unix domain socket
if (($socket = socket_create(AF_UNIX, SOCK_STREAM, 0)) === false) 
	{
    echo "socket_create() failed: reason: " . socket_strerror(socket_last_error()) . "";
	}

if (socket_connect($socket, $socket_file) === false) 
	{
    echo "socket_connect() failed: reason: " . socket_strerror(socket_last_error($socket)) . "";
	}

$msg = 'Your message here';
$write_res = socket_write($socket, $msg, strlen($msg));
if ($write_res === false)
	{ echo 'Socket write error: ' . socket_strerror(socket_last_error($socket)) . ''; } 
else 
	{
    echo 'Message sent successfully';
	}

socket_close($socket);


?>
