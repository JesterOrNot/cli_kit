var resourcesSuffix = "";
var currentTheme = document.getElementById("themeStyle");
var mainTheme = document.getElementById("mainThemeStyle");
var savedHref = [];
function hasClass(elem, className) {
  return elem && elem.classList && elem.classList.contains(className)
}
function addClass(elem, className) {
  if (!elem || !elem.classList) {
    return
  }
  elem.classList.add(className)
}
function removeClass(elem, className) {
  if (!elem || !elem.classList) {
    return
  }
  elem.classList.remove(className)
}
function isHidden(elem) { return elem.offsetParent === null }
function onEach(arr, func, reversed) {
  if (arr && arr.length > 0 && func) {
    var length = arr.length;
    if (reversed !== true) {
      for (var i = 0; i < length; ++i) {
        if (func(arr[i]) === true) {
          return true
        }
      }
    } else {
      for (var i = length - 1; i >= 0; --i) {
        if (func(arr[i]) === true) {
          return true
        }
      }
    }
  }
  return false
}
function onEachLazy(lazyArray, func, reversed) {
  return onEach(Array.prototype.slice.call(lazyArray), func, reversed)
}
function usableLocalStorage() {
  if (typeof Storage === "undefined") {
    return false
  }
  try {
    return window.localStorage !== null && window.localStorage !== undefined
  } catch (err) {
    return false
  }
}
function updateLocalStorage(name, value) {
  if (usableLocalStorage()) {
    localStorage[name] = value
  } else {
  }
}
function getCurrentValue(name) {
  if (usableLocalStorage() && localStorage[name] !== undefined) {
    return localStorage[name]
  }
  return null
}
function switchTheme(styleElem, mainStyleElem, newTheme, saveTheme) {
  var fullBasicCss = "rustdoc" + resourcesSuffix + ".css";
  var fullNewTheme = newTheme + resourcesSuffix + ".css";
  var newHref = mainStyleElem.href.replace(fullBasicCss, fullNewTheme);
  if (styleElem.href === newHref) {
    return
  }
  var found = false;
  if (savedHref.length === 0) {
    onEachLazy(document.getElementsByTagName("link"),
               function(el) { savedHref.push(el.href) })
  }
  onEach(savedHref, function(el) {
    if (el === newHref) {
      found = true;
      return true
    }
  });
  if (found === true) {
    styleElem.href = newHref;
    if (saveTheme === true) {
      updateLocalStorage("rustdoc-theme", newTheme)
    }
  }
}
function getSystemValue() {
  var property =
      getComputedStyle(document.documentElement).getPropertyValue('content');
  return property.replace(/[\"\']/g, "")
}
switchTheme(currentTheme, mainTheme,
            getCurrentValue("rustdoc-theme") || getSystemValue() || "light",
            false)