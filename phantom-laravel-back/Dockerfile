From php:8.2-apache 

#------------------------------------#
# Instalar dependencias
#------------------------------------#

WORKDIR /var/www/html

RUN docker-php-ext-install pdo pdo_mysql mysql 
COPY . /var/www/html

RUN chown -R www-data:www-data /var/www/html && \
    chmod -R 755 /var/www/html 

#------------------------------------#
# Instalar Composer
#------------------------------------#
RUN apt-get update && apt-get install -y curl git unzip && \
    curl -sS https://getcomposer.org/installer | php -- --install-dir=/usr/local/bin --filename=composer && \
    composer install --no-interaction --optimize-autoloader

#------------------------------------#
# Expoer el puerto 80
#------------------------------------#
EXPOSE 80
