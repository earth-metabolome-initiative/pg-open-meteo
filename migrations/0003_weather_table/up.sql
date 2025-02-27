CREATE TABLE sample_weather_data (
    weather_id INT PRIMARY KEY NOT NULL,
    location INT NOT NULL, -- Foreign key to position.id
    requested_at TIMESTAMP,  -- When we fetched the data
    temperature_2m FLOAT, -- default unit of API : °C
    relative_humidity_2m FLOAT, -- percentage : range [0,1]
    dew_point_2m FLOAT, -- Unit: °C, Dew point temperature at 2 meters above ground
    apparent_temperature FLOAT, -- Unit °C
    surface_pressure FLOAT, -- Unit : hPa

    FOREIGN KEY (location) REFERENCES position(id)
);