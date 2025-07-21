import React from 'react';
import { Card, CardContent } from '@mui/material';
import { useAnimations, animationStyles } from '../../utils/fantasyAnimations';

const FantasyCard = ({ 
  children, 
  animation = 'fadeInUp', 
  delay = 0,
  colorScheme = 'primary',
  floating = true,
  glassMorphism = false,
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
      case 'rotateIn':
        return animationStyles.rotateIn(delay);
      default:
        return {};
    }
  };

  const getColorGradient = () => {
    const gradients = {
      primary: 'rgba(33, 150, 243, 0.1)',
      secondary: 'rgba(156, 39, 176, 0.1)',
      success: 'rgba(76, 175, 80, 0.1)',
      warning: 'rgba(255, 152, 0, 0.1)',
      info: 'rgba(0, 188, 212, 0.1)',
    };
    return gradients[colorScheme] || gradients.primary;
  };

  return (
    <Card
      sx={{
        ...getAnimationStyle(),
        ...(floating ? animationStyles.floatingCard : {}),
        ...(glassMorphism ? animationStyles.glassMorphism : {}),
        background: glassMorphism 
          ? animationStyles.glassMorphism.background 
          : `linear-gradient(145deg, ${getColorGradient()}, rgba(255,255,255,0.05))`,
        backdropFilter: glassMorphism ? 'blur(10px)' : 'none',
        border: glassMorphism ? '1px solid rgba(255,255,255,0.2)' : 'none',
        borderRadius: '16px',
        overflow: 'hidden',
        position: 'relative',
        '&::before': {
          content: '""',
          position: 'absolute',
          top: 0,
          left: 0,
          right: 0,
          height: '4px',
          background: `linear-gradient(90deg, ${
            colorScheme === 'primary' ? '#2196F3, #21CBF3' :
            colorScheme === 'secondary' ? '#9C27B0, #E1BEE7' :
            colorScheme === 'success' ? '#4CAF50, #81C784' :
            colorScheme === 'warning' ? '#FF9800, #FFB74D' :
            '#00BCD4, #4DD0E1'
          })`,
          animation: 'gradientShift 3s ease infinite',
        },
        ...sx,
      }}
      {...props}
    >
      {children}
    </Card>
  );
};

export default FantasyCard;
