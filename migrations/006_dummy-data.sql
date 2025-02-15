INSERT INTO users
    (id, email, hash_password, name, handle, profile_img_url, bio, is_profile_complete, is_deleted, deleted_at, created_at, updated_at)
VALUES
    (
        1,
        'test.user.0@email.com',
        '$2b$13$mzdLWfR.EQM915IxmyG2s.L.6gtymPrELlZncVAI.ttzLoaXYP3he',
        'Elon Musk',
        '@elonmusk',
        'https://upload.wikimedia.org/wikipedia/commons/c/cb/Elon_Musk_Royal_Society_crop.jpg',
        '',
        1,
        0,
        NULL,
        '2025-02-08 13:13:00',
        '2025-02-08 13:19:27'
    );

INSERT INTO users
    (id, email, hash_password, name, handle, profile_img_url, bio, is_profile_complete, is_deleted, deleted_at, created_at, updated_at)
VALUES
    (
        2,
        'test.user.1@email.com',
        '$2b$13$sr2Dy2lVImHjrTJwwIGrX.rCnsEyYAhy.MTAFw7zWBzAnAo24Z24C',
        'Ted Song',
        '@tedsong',
        '',
        '',
        1,
        0,
        NULL,
        '2025-02-08 13:13:34',
        '2025-02-08 13:13:34'
    );

INSERT INTO users
    (id, email, hash_password, name, handle, profile_img_url, bio, is_profile_complete, is_deleted, deleted_at, created_at, updated_at)
VALUES
    (
        3,
        'test.user.2@email.com',
        '$2b$13$9o6o7pe1l51iff1y.2uYJu4aeknfVa2SYXDEzr7GMZkoIhl4VYh7K',
        'Andrew Ng',
        '@andrewNg',
        'https://aifund.ai/wp-content/uploads/2021/08/Team-PhotosAndrew-NG.jpg',
        'Hello',
        1,
        0,
        NULL,
        '2025-02-08 13:14:34',
        '2025-02-08 13:14:34'
    );

INSERT INTO follow (user_id, follower_id)
VALUES 
    (1, 2),  
    (1, 3),  
    (2, 3),  
    (3, 1);  

INSERT INTO thread (user_id, title, content, parent_thread, is_deleted)
VALUES
    (1, 'First Thread', 'This is the first thread content.', NULL, 0),
    (2, 'Second Thread', 'This is the second thread content.', NULL, 0),
    (3, 'Reply to First', 'This is a reply to the first thread.', 1, 0),
    (1, 'Third Thread', 'This is the third thread content', NULL, 0),
    (1, 'Deleted Thread', 'This is the deleted thread content', NULL, 1);

INSERT INTO upvote (user_id, thread_id, reaction)
VALUES
    (1, 1, 'UP'),   
    (2, 1, 'DOWN'), 
    (3, 2, 'UP'),   
    (1, 3, 'UP');   


INSERT INTO views (thread_id, view_count)
VALUES
    (1, 10), 
    (2, 5),  
    (3, 7);  
