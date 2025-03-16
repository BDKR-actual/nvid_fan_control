<?php


$fp = fsockopen("unix:///tmp/chart_comm");


$command = '
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
The command is 1 The command is 1 The command is 1 The command is 1
';


$fwr_res = fwrite($fp, $command, mb_strlen($command, 'UTF8') );

print_r($fp);
echo "------------------------------------------------------------------\n";
print_r($fwr_res);


?>
