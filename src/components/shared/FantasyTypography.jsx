import React from 'react';
import { Typography } from '@mui/material';
import { useAnimations, animationStyles } from '../../utils/fantasyAnimations';

const FantasyTypography = ({ 
  children, 
  variant = 'body1',
  gradient = false,
  colorScheme = 'primary',
  animation = 'fadeInUp',
  delay = 0,
  sx = {},
  ...props 
}) => {
  useAnimations();

  const getAnimationStyle = () => {
    switch (animation) {
      case 'fadeInUp':
        return animationStyles.fadeInUp(delay);
      case 'fadeInDown':
        return animationStyles.fadeInDown(delay);
      case 'slideInLeft':
        return animationStyles.slideInLeft(delay);
      case 'slideInRight':
        return animationStyles.slideInRight(delay);
      case 'scaleIn':
        return animationStyles.scaleIn(delay);
      default:
        return {};
    }
  };

  const getGradientColors = () => {
    const colors = {
      primary: ['#2196F3', '#21CBF3'],
      secondary: ['#9C27B0', '#E1BEE7'],
      success: ['#4CAF50', '#81C784'],
      warning: ['#FF9800', '#FFB74D'],
      info: ['#00BCD4', '#4DD0E1'],
    };
    return colors[colorScheme] || colors.primary;
  };

  return (
    <Typography
      variant={variant}
      sx={{
        ...getAnimationStyle(),
        ...(gradient ? animationStyles.gradientText(getGradientColors()) : {}),
        fontWeight: ['h1', 'h2', 'h3', 'h4', 'h5', 'h6'].includes(variant) ? 'bold' : 'inherit',
        ...sx,
      }}
      {...props}
    >
      {children}
    </Typography>
  );
};

export default FantasyTypography;
