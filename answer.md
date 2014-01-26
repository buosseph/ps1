Title: Problem Set 1 Answers
Author: Brian Uosseph

1. User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/32.0.1700.77 Safari/537.36

	Mozilla/5.0, shows that the browser is "Mozilla-compatible" running version 5.0 of Mozilla. (Macintosh; Intel Mac OS X 10_9_1) shows the platform the browser is currently running and indicates what CPU and operating system the platform is using. AppleWebKit/537.36 indicates what webkit, and its build, is provided for basic display of web content in the browser. (KHTML, like Gecko) shows that the browser uses the Open Source HTML layout engine developed by the KDE project and works similarly to Gecko... Chrome/32.0.1700.77 is the name of the browser application being used to access the server and the version of the browser. Safari/537.36 indicates that the browser is based on the that build of Safari.

2.	In Rust, static items are designed to be constants as they are evaluted at compile time. In cases where multiple functions in a program are using the static variable as a parameter, it's possible that one function changes the variabe so that another function does not recognize the new value, and potentially crash the program, or the second function does not recognize the new value before the frist function compeletes the change.
