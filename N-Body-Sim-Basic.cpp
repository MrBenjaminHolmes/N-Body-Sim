#include <iostream>
#include <SFML/Graphics.hpp>
using namespace std;

const float G = 1.0f;
const float dt = 0.01f;
class Body
{
public:
    Body(float mass, float radius, sf::Vector2f pos, sf::Vector2f vel)
        : mass(mass),
        radius(radius),
        pos(pos),
        vel(vel),
        shape(radius)
    {
        shape.setFillColor(sf::Color::Red);
        shape.setOrigin(sf::Vector2f(radius, radius));
        shape.setPosition(pos);
    }

    float mass;
    float radius;
    sf::Vector2f pos;
    sf::Vector2f vel;
    sf::CircleShape shape;

    void update(Body& partner)
    {
        sf::Vector2f direction = partner.pos - pos;

        float distance = sqrt(direction.x * direction.x +
            direction.y * direction.y);

        if (distance < 1.0f)
            return;

        sf::Vector2f unitDirection = direction / distance;

        float forceMagnitude =
            G * mass * partner.mass / (distance * distance);

        sf::Vector2f force = unitDirection * forceMagnitude;

        sf::Vector2f accel = force / mass;

        vel += accel * dt;
        pos += vel * dt;

        shape.setPosition(pos);
    }

};

int main()
{

    vector<Body> objects = {};
    sf::RenderWindow window(sf::VideoMode({ 1000, 1000 }), "N-Body Simulation");
    sf::View mainView;
    mainView = sf::View(sf::FloatRect({ 0.0f, 0.0f }, { 1000.0f, 1000.0f }));

    //------Basic Orbit Setup-----//
    Body obj1(1000.0f, 10.0f,
        sf::Vector2f(500, 500),
        sf::Vector2f(0, 0));

    Body obj2(1.0f, 4.0f,
        sf::Vector2f(700, 500),
        sf::Vector2f(0, -2.236f));
    objects.push_back(obj1);
    objects.push_back(obj2);

    //-----------------------------//


    while (window.isOpen())
    {
        while (const std::optional event = window.pollEvent())
        {
            if (event->is<sf::Event::Closed>())
                window.close();

            if (const auto* wheel =
                event->getIf<sf::Event::MouseWheelScrolled>())
            {

                if (wheel->delta > 0)
                    mainView.zoom(0.9f);
                else
                    mainView.zoom(1.1f);
            }
            if (const auto* mousePressed = event->getIf<sf::Event::MouseButtonPressed>())
            {
                if (mousePressed->button == sf::Mouse::Button::Left)
                {

                    std::cout << "Mouse x: " << mousePressed->position.x << '\n';
                    std::cout << "Mouse y: " << mousePressed->position.y << '\n';
                    Body objn(0.01f, 4.0f,
                        sf::Vector2f(mousePressed->position.x, mousePressed->position.y),
                        sf::Vector2f(0, 0));
                    objects.push_back(objn);
                }
            }
        }
        window.clear();
        window.setView(mainView);

        for (Body& obj : objects)
        {
            window.draw(obj.shape);

            for (Body& partnerObj : objects)
            {
                if (&obj == &partnerObj)
                    continue;

                obj.update(partnerObj);
            }
        }

        window.display();

    }
}
