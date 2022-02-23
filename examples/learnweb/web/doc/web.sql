-- Create Table
-- Drop table
-- DROP TABLE public."user";
CREATE TABLE public."user" (
	usercode varchar NULL,
	username varchar NULL,
	useremail varchar NULL
);

-- Insert into public."user" row data
INSERT INTO public."user" (usercode, username, useremail) VALUES('12345', '姜坤', 'jiangkun@livstyle.cn');
