-- Add migration script here

INSERT INTO users (user_id, username, password_hash)
VALUES (
	'ddf8994f-d522-4659-8d02-c1d479057be6',
	'admin',
	'$argon2id$v=19$m=1500,t=2,p=1$Nw5PiOSB089EWO7S0oDQBA$jyprKkiSpKzATlyVrz96GmmdtE3hUyHb1a2PxgfyeQw'
	)
