-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS jobs (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    company VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    date VARCHAR(255) NOT NULL,
    description VARCHAR(1000) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    title VARCHAR(255) NOT NULL,
    description VARCHAR(1000) NOT NULL,
    imgs VARCHAR(10000) [] NOT NULL,
    demo VARCHAR(500) NOT NULL,
    git VARCHAR(500) NOT NULL,
    stacks VARCHAR(500) [] NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS testimonials (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    name VARCHAR(255) NOT NULL,
    comment VARCHAR(1000) NOT NULL,
    position VARCHAR(255) NOT NULL,
    company VARCHAR(255) NOT NULL,
    img VARCHAR(500) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS details (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    title VARCHAR(1000) NOT NULL,
    logo VARCHAR(1000) NOT NULL,
    keywords VARCHAR(1000) NOT NULL,
    site_description VARCHAR(1000) NOT NULL,
    description VARCHAR(1000) NOT NULL,
    about VARCHAR(1000) NOT NULL,
    position VARCHAR(255) NOT NULL,
    company VARCHAR(255) NOT NULL,
    img VARCHAR(500) NOT NULL,
    github VARCHAR(500) NOT NULL,
    linkedin VARCHAR(500) NOT NULL,
    email VARCHAR(500) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS images (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    name VARCHAR(1000) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    username VARCHAR(1000) NOT NULL,
    password VARCHAR(1000) NOT NULL,
    email VARCHAR(1000) NOT NULL,
    fullname VARCHAR(1000) NOT NULL,
    role SMALLINT NOT NULL DEFAULT 1,
    avatar VARCHAR(1000) NOT NULL,
    notes VARCHAR(1000) NOT NULL,
    active SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS task_supervisors (
    supervisor_id SERIAL PRIMARY KEY,
    task_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS task_visors (
    visor_id SERIAL PRIMARY KEY,
    task_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS task_updates (
    update_id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    text VARCHAR(10000) NOT NULL, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS tasks (
    task_id SERIAL PRIMARY KEY,
    group_id INTEGER NOT NULL,
    name VARCHAR(10000) NOT NULL,
    date TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expiration_date TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    status SMALLINT NOT NULL DEFAULT 1,
    priority SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS task_groups (
    group_id SERIAL PRIMARY KEY,
    agenda_id INTEGER NOT NULL,
    title VARCHAR(1000) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS agendas (
    agenda_id SERIAL PRIMARY KEY,
    title VARCHAR(1000) NOT NULL,
    owner_id INTEGER NOT NULL,    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS user_roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(1000) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

INSERT INTO
    jobs (company, title, date, description)
VALUES
    (
        'Kalenux',
        'Founder',
        '2020 - 2024',
        'Founded Kalenux company for operating software business'
    );

INSERT INTO
    projects (title, description, imgs, demo, git, stacks)
VALUES
    (
        'Kalenuxer',
        'A website building framework that provides all the necessary tools for building highly efficient for performance web applications, It contains multi language support, minifiers, obfuscators, SSR, templaters (mail, section, pages..), classification, svg to icons, versionizers, css, js structure and localizations,',
        ARRAY [ 'kalenuxer.webp' ],
        '',
        'https://github.com/kalinux0/Kalenuxer',
        ARRAY [ 'Modifiable Templating Language', 'HTML/HTML5', 'CSS/CSS3', 'Javascript', 'PHP', 'Node.js', 'LAMP', 'AWS', 'Git', 'Figma', 'MySQL' ]
    ),
    (
        'My Partners Law Firm',
        'A website and control panel that i built for a Law Firm it also contains SSR algorithms that converts the SQL datas to HTML structure with Kalenuxer',
        ARRAY [ 'myp_1.webp', 'myp_2.webp', 'myp_3.webp', 'myp_4.webp', 'myp_5.webp', 'myp_6.webp', 'myp_7.webp', 'myp_8.webp', 'myp_9.webp', 'myp_10.webp', 'myp_11.webp', 'myp_12.webp' ],
        'https://myp.emirbaycan.com.tr',
        '',
        ARRAY [ 'Kalenuxer', 'HTML/HTML5', 'CSS/CSS3', 'Javascript', 'Node.js', 'PHP', 'MySQL' ]
    ),
    (
        'Hukuki Yeterlilik Akademisi',
        'A website and control panel that i built for an Education Services Firm with Kalenuxer',
        ARRAY [ 'hya_1.webp', 'hya_2.webp', 'hya_3.webp', 'hya_4.webp', 'hya_5.webp', 'hya_6.webp', 'hya_7.webp', 'hya_8.webp', 'hya_9.webp', 'hya_10.webp', 'hya_11.webp', 'hya_12.webp' ],
        'https://hya.emirbaycan.com.tr',
        '',
        ARRAY [ 'Kalenuxer', 'HTML/HTML5', 'CSS/CSS3', 'Javascript', 'Node.js', 'PHP', 'MySQL' ]
    ),
    (
        'Morkoç Law Firm',
        'A website and control panel that i built for an Education Services Firm with Kalenuxer',
        ARRAY [ 'morkoc_1.webp', 'morkoc_2.webp', 'morkoc_3.webp', 'morkoc_4.webp', 'morkoc_5.webp', 'morkoc_6.webp', 'morkoc_7.webp', 'morkoc_8.webp', 'morkoc_9.webp', 'morkoc_10.webp', 'morkoc_11.webp', 'morkoc_12.webp' ],
        'https://morkoc.emirbaycan.com.tr',
        '',
        ARRAY [ 'Kalenuxer', 'HTML/HTML5', 'CSS/CSS3', 'Javascript', 'Node.js', 'PHP', 'MySQL' ]
    ),
    (
        'Karalar Prefabric',
        'A website and control panel that i built for a Prefabric House Firm with Kalenuxer',
        ARRAY [ 'kp_1.webp', 'kp_2.webp', 'kp_3.webp', 'kp_4.webp', 'kp_5.webp', 'kp_6.webp', 'kp_7.webp', 'kp_8.webp', 'kp_9.webp', 'kp_10.webp', 'kp_11.webp', 'kp_12.webp', 'kp_13.webp', 'kp_14.webp' ],
        'https://kp.emirbaycan.com.tr',
        '',
        ARRAY [ 'Kalenuxer', 'HTML/HTML5', 'CSS/CSS3', 'Javascript', 'Node.js', 'PHP', 'MySQL' ]
    ),
    (
        'Eyüp Sultan Tulumbacısı',
        'A website that i built for a Dessert Firm with Kalenuxer',
        ARRAY [ 'est_1.webp', 'est_2.webp', 'est_3.webp', 'est_4.webp', 'est_5.webp', 'est_6.webp', 'est_7.webp' ],
        'https://est.emirbaycan.com.tr',
        '',
        ARRAY [ 'Kalenuxer', 'HTML/HTML5', 'CSS/CSS3', 'Javascript', 'Node.js', 'PHP', 'MySQL' ]
    ),
    (
        'Girişimci Hukukçular Derneği',
        'The first website and control panel that i built for a Law Society with LAMP stack',
        ARRAY [ 'ghd_1.webp', 'ghd_2.webp', 'ghd_3.webp', 'ghd_4.webp', 'ghd_5.webp' ],
        'https://ghd.emirbaycan.com.tr',
        '',
        ARRAY [ 'HTML/HTML5', 'CSS/CSS3', 'Javascript', 'PHP', 'MySQL' ]
    ),
    (
        'Hukuk Eğitim Programları',
        'A website and control panel that i built and continuing to develop for my friends with Kalenuxer',
        ARRAY [ 'hep_1.webp', 'hep_2.webp', 'hep_3.webp', 'hep_4.webp', 'hep_5.webp', 'hep_6.webp', 'hep_7.webp', 'hep_8.webp', 'hep_9.webp' ],
        'https://hep.emirbaycan.com.tr',
        '',
        ARRAY [ 'HTML/HTML5', 'CSS/CSS3', 'Javascript', 'PHP', 'MySQL' ]
    ),
    (
        'Do Eloboost',
        'The first website and control panel that i built and continuing to develop for my friends with Kalenuxer',
        ARRAY [ 'de.webp' ],
        '',
        '',
        ARRAY [ 'Kalenuxer', 'HTML/HTML5', 'CSS/CSS3', 'Javascript', 'PHP', 'MySQL' ]
    ),
    (
        'Terapi Kliniği',
        'The first website and control panel that i built for a Psychologist Firm with Laravel i also adapted the Laravel for running on a hosting provider and coded many javascript libraries for this project',
        ARRAY [ 'tk.webp' ],
        '',
        '',
        ARRAY [ 'LAMP', 'HTML/HTML5', 'CSS/CSS3', 'Javascript', 'PHP', 'Laravel', 'Composer', 'MySQL' ]
    ),
    (
        'Murat Bulat Law Firm',
        'The first website and control panel that i built for a Lawyer Firm with LAMP stack',
        ARRAY [ 'mb.webp' ],
        '',
        '',
        ARRAY [ 'LAMP', 'HTML/HTML5', 'CSS/CSS3', 'Javascript', 'PHP', 'MySQL' ]
    ),
    (
        'Portfolio w Kalenuxer',
        'I built some multi language portfolio websites as hobby for me with different frameworks',
        ARRAY [ 'lamp_1.webp', 'lamp_2.webp', 'lamp_3.webp', 'lamp_4.webp', 'lamp_5.webp' ],
        'https://lamp.emirbaycan.com.tr',
        '',
        ARRAY [ 'Kalenuxer', 'LAMP' ]
    ),
    (
        'Portfolio w React',
        'I built some multi language portfolio websites as hobby for me with different frameworks',
        ARRAY [ 'mern_1.webp', 'mern_2.webp', 'mern_3.webp', 'mern_4.webp', 'mern_5.webp', 'mern_6.webp' ],
        'https://mern.emirbaycan.com.tr',
        '',
        ARRAY [ 'Mysql', 'React', 'Node.js', 'Apache', 'Ubuntu' ]
    ),
    (
        'Portfolio w Laravel & React',
        'I built some multi language portfolio websites as hobby for me with different frameworks',
        ARRAY [ 'nmlr_1.webp', 'nmlr_2.webp', 'nmlr_3.webp', 'nmlr_4.webp' ],
        'https://nmlr.emirbaycan.com.tr',
        '',
        ARRAY [ 'Three.js', 'Laravel', 'React', 'Node.js', 'Apache', 'Ubuntu' ]
    ),
    (
        'Portfolio w Nextjs',
        'I built some multi language portfolio websites as hobby for me with different frameworks',
        ARRAY [ 'next_1.webp', 'next_2.webp', 'next_3.webp', 'next_4.webp' ],
        'https://next.emirbaycan.com.tr',
        '',
        ARRAY [ 'Next.js', 'Resend', 'Node.js', 'Apache', 'Ubuntu' ]
    ),
    (
        'Account Creator',
        'An account creator built with purpose of creating the accounts on social platforms without human interactions using vpns, proxies',
        ARRAY [ 'account_creator.webp' ],
        '',
        '',
        ARRAY [ 'Firefox build', 'C#', 'Selenium', 'Geckofx', 'AWS', 'C#', 'Google Cloud', 'Azure' ]
    ),
    (
        'Voter',
        'An human data creator with using Selenium and Tor Browser ips',
        ARRAY [ 'voter.webp' ],
        '',
        '',
        ARRAY [ 'Python', 'Tor Browser', 'Selenium' ]
    ),
    (
        'Height and Speed Calculator',
        'Simple python algorithm that calculates the height of the subject given',
        ARRAY [ 'height_speed_calculator.webp' ],
        '',
        '',
        ARRAY [ 'Python', 'RasperryPi', 'Servo' ]
    );

INSERT INTO
    details (
        title,
        logo,
        keywords,
        site_description,
        description,
        about,
        position,
        company, 
        img,
        github,
        linkedin,
        email
    )
VALUES
    (
        'Emir Baycan',
        'logo.webp',
        'Emir Baycan, web developer, software engineer',
        'Results-driven Senior Software Developer base in Turkey with over 5 years of professional experience in web and software development. I have worked on various large-scale projects that prioritized responsive design, performance optimization, and cross-functional collaboration. I pride myself on my ability to translate project requirements into visually appealing and efficient solutions.',
        'I''m a software engineer with <span class=''underline''>over 5 years of experience</span>, specializing in developing systems, interfaces, bots, and technological solutions. I pride myself on my ability to translate project requirements into visually appealing and efficient solutions.',
        'Hey there, I''m Emir Baycan. I hold long variety of skills and currently exploring the world of Mobile App Design and Development. Throughout my career, I''ve been involved in various large-scale projects, prioritizing responsive design, performance optimization, and cross-functional collaboration. I love turning a project''s ''wish list'' into something that not only looks great but also performs well.',
        'Freelancer',
        'Kalenux',
        'me.webp',
        'https://github.com/kalinux0',
        'https://www.linkedin.com/in/emirbaycan/',
        'emir-baycan@hotmail.com'
    );

INSERT INTO
    users (
        username,
        password,
        email,
        fullname,
        role,
        avatar,
        notes,
        active
    )
VALUES
    (
        'admin',
        '$2b$12$wC2.kKbuZ9EnAm52GsJfv.U3mxBAxdLqvuP0aTgdnW3UMjm6Nu466',
        'emir-baycan@hotmail.com',
        'Emir Baycan',
        3,
        'me.webp',
        'Owner',
        1
    );

INSERT INTO
    user_roles (name)
VALUES
    ('New User'),
    ('User'),
    ('Admin');