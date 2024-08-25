
import './index.css'
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuList,
  NavigationMenuTrigger,
} from "@/components/ui/navigation-menu"

let page = "Login";


function App() {
  if (page=="Login"){
      return (
        <>
          <NavigationMenu>
            <NavigationMenuList>
              <NavigationMenuItem>
                <NavigationMenuTrigger>Forms</NavigationMenuTrigger>
                <NavigationMenuTrigger>Statistics</NavigationMenuTrigger>
              </NavigationMenuItem>
            </NavigationMenuList>
          </NavigationMenu>
        </>
      )
}

}
export default App
