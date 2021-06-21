function clickUrl(url) {

  const redirect = "https://click.findereport.com/click?url="
  const link = redirect + url
  
  $.get(link, function(data, status) {
    console.log(`${status}`)
  });
}

$(document).on("click", ".external", function (e) {
  
    // stop browser from going to href right away
    e.preventDefault(); 
  
    const redirect = "https://click.findereport.com/click?url="
    let externalUrl = this.href;
    let link = redirect + externalUrl;
    
    $.ajax({
        url: link,
        success: function () {
            location.href = externalUrl;
        },
        error: function () {
            location.href = externalUrl;
        }
    });
});

$("#search_icon").click(function(){
  $("#search_field").toggle();
  $("#search_field").focus();
  $("#search_field").setCursorPosition(0);
});

clickUrl(window.location)
