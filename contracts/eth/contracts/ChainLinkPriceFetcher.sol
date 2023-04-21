pragma solidity ^0.8.7;

import "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol";

contract ChainLinkPriceFetcher {

    struct PriceDetails{
        int256 price;
        uint256 fetchedAt;
    }
    //index => chainlink address
    mapping (uint256 => address) public priceFeed;
    mapping(uint256 => PriceDetails) public latestPrice;
    uint256 public minBlockDelayAllowed;

    constructor(uint256[] memory indices, address[] memory feedAddress, uint256 minBlockDelayAllowed_) {
        require(indices.length == feedAddress.length, "length dont match");
        for (uint i = 0; i < indices.length; i++){
            priceFeed[indices[i]] = feedAddress[i];
        }
        minBlockDelayAllowed = minBlockDelayAllowed_;
    }

    function fetchPrice(uint256 index) external returns(int){
        require(priceFeed[index] != address(0), "index not configured");
        PriceDetails memory priceDetails = latestPrice[index];
        if (block.number - priceDetails.fetchedAt >= minBlockDelayAllowed){
            (,int price, , ,) = AggregatorV3Interface(priceFeed[index]).latestRoundData();
            latestPrice[index] = PriceDetails(price, block.number);
            return price;
        }
        return priceDetails.price;
    }
}