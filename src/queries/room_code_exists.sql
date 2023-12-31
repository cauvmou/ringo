SELECT COUNT(pk_room_code) AS 'NUM MATCHING' FROM room
WHERE pk_room_code = :code;