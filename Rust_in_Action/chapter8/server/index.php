<?php

$val = "Hello, ";

if(isset($_GET['name'])) {
	$val = $val.$_GET['name'];
} else {
	$val = $val."world!";
}

echo "<div> ".$val." </div>";
